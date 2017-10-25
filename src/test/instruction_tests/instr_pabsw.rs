use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 229], OperandSize::Dword)
}

#[test]
fn pabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(ESI, 1953151290, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 190, 58, 185, 106, 116], OperandSize::Dword)
}

#[test]
fn pabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 207], OperandSize::Qword)
}

#[test]
fn pabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 16], OperandSize::Qword)
}

#[test]
fn pabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 198], OperandSize::Dword)
}

#[test]
fn pabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1180122975, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 132, 134, 95, 63, 87, 70], OperandSize::Dword)
}

#[test]
fn pabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 228], OperandSize::Qword)
}

#[test]
fn pabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 44, 115], OperandSize::Qword)
}

