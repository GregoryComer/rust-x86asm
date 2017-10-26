use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 223], OperandSize::Dword)
}

#[test]
fn psignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 47], OperandSize::Dword)
}

#[test]
fn psignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 254], OperandSize::Qword)
}

#[test]
fn psignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 113762929, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 156, 254, 113, 226, 199, 6], OperandSize::Qword)
}

#[test]
fn psignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 237], OperandSize::Dword)
}

#[test]
fn psignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDI, 927331757, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 159, 173, 245, 69, 55], OperandSize::Dword)
}

#[test]
fn psignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 201], OperandSize::Qword)
}

#[test]
fn psignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 280148068, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 142, 100, 184, 178, 16], OperandSize::Qword)
}

