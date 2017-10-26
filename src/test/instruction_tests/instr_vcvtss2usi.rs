use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 24, 121, 203], OperandSize::Dword)
}

#[test]
fn vcvtss2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(EBP)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 42], OperandSize::Dword)
}

#[test]
fn vcvtss2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 126, 56, 121, 216], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 121, 38], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 254, 56, 121, 218], OperandSize::Qword)
}

#[test]
fn vcvtss2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2USI, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 840680321, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 121, 172, 198, 129, 195, 27, 50], OperandSize::Qword)
}

