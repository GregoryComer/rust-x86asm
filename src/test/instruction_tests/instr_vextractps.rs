use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 254, 58], OperandSize::Dword)
}

#[test]
fn vextractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 62, 47], OperandSize::Dword)
}

#[test]
fn vextractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 205, 76], OperandSize::Qword)
}

#[test]
fn vextractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectDisplaced(RCX, 923630140, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 161, 60, 122, 13, 55, 94], OperandSize::Qword)
}

#[test]
fn vextractps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 199, 15], OperandSize::Dword)
}

#[test]
fn vextractps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 43, 42], OperandSize::Dword)
}

#[test]
fn vextractps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 210, 59], OperandSize::Qword)
}

#[test]
fn vextractps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM30)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 8, 23, 52, 219, 114], OperandSize::Qword)
}

