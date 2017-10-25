use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn minps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 245], OperandSize::Dword)
}

fn minps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1095585417, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 60, 189, 137, 78, 77, 65], OperandSize::Dword)
}

fn minps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 214], OperandSize::Qword)
}

fn minps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RAX, 391264117, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 160, 117, 55, 82, 23], OperandSize::Qword)
}

