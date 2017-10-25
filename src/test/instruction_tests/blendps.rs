use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn blendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 200, 81], OperandSize::Dword)
}

fn blendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 23, 109], OperandSize::Dword)
}

fn blendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 220, 63], OperandSize::Qword)
}

fn blendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RAX, 1813226492, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 128, 252, 163, 19, 108, 25], OperandSize::Qword)
}

