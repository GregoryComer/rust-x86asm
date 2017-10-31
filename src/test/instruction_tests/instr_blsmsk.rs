use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsmsk_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 243, 214], OperandSize::Dword)
}

#[test]
fn blsmsk_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 23], OperandSize::Dword)
}

#[test]
fn blsmsk_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 214], OperandSize::Qword)
}

#[test]
fn blsmsk_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1276483359, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 20, 213, 31, 151, 21, 76], OperandSize::Qword)
}

#[test]
fn blsmsk_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RDX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 232, 243, 214], OperandSize::Qword)
}

#[test]
fn blsmsk_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 243, 20, 143], OperandSize::Qword)
}

