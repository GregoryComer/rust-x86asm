use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sha1msg1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 221], OperandSize::Dword)
}

fn sha1msg1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 1001258575, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 128, 79, 254, 173, 59], OperandSize::Dword)
}

fn sha1msg1_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 246], OperandSize::Qword)
}

fn sha1msg1_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1MSG1, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 1175141819, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 201, 185, 187, 61, 11, 70], OperandSize::Qword)
}

