use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vinsertf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 24, 192, 17], OperandSize::Dword)
}

fn vinsertf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 353225365, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 24, 156, 186, 149, 202, 13, 21, 81], OperandSize::Dword)
}

fn vinsertf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 24, 194, 36], OperandSize::Qword)
}

fn vinsertf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTF128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RAX, 1337666297, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 24, 152, 249, 42, 187, 79, 119], OperandSize::Qword)
}

