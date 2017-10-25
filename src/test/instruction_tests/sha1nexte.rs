use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sha1nexte_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 207], OperandSize::Dword)
}

fn sha1nexte_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1878744270, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 20, 125, 206, 92, 251, 111], OperandSize::Dword)
}

fn sha1nexte_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 226], OperandSize::Qword)
}

fn sha1nexte_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA1NEXTE, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 580853945, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 200, 142, 185, 32, 159, 34], OperandSize::Qword)
}

