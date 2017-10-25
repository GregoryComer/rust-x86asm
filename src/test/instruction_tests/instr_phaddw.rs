use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 223], OperandSize::Dword)
}

#[test]
fn phaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 62], OperandSize::Dword)
}

#[test]
fn phaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 238], OperandSize::Qword)
}

#[test]
fn phaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 36, 192], OperandSize::Qword)
}

#[test]
fn phaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 254], OperandSize::Dword)
}

#[test]
fn phaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 852868097, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 180, 219, 1, 188, 213, 50], OperandSize::Dword)
}

#[test]
fn phaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 251], OperandSize::Qword)
}

#[test]
fn phaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1625485077, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 44, 245, 21, 239, 226, 96], OperandSize::Qword)
}

