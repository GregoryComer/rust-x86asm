use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 236, 89], OperandSize::Dword)
}

#[test]
fn vextractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1101618962, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 44, 125, 18, 95, 169, 65, 99], OperandSize::Dword)
}

#[test]
fn vextractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 252, 90], OperandSize::Qword)
}

#[test]
fn vextractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 35, 84], OperandSize::Qword)
}

#[test]
fn vextractps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 236, 25], OperandSize::Dword)
}

#[test]
fn vextractps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 60, 134, 97], OperandSize::Dword)
}

#[test]
fn vextractps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM30)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 8, 23, 246, 113], OperandSize::Qword)
}

#[test]
fn vextractps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM26)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 8, 23, 20, 195, 31], OperandSize::Qword)
}

