use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 229], OperandSize::Dword)
}

#[test]
fn phaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 12, 198], OperandSize::Dword)
}

#[test]
fn phaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 252], OperandSize::Qword)
}

#[test]
fn phaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 1, 20, 83], OperandSize::Qword)
}

#[test]
fn phaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 243], OperandSize::Dword)
}

#[test]
fn phaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 63], OperandSize::Dword)
}

#[test]
fn phaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 230], OperandSize::Qword)
}

#[test]
fn phaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 1, 12, 182], OperandSize::Qword)
}

