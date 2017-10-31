use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 187, 234], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 187, 36, 247], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 187, 216], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RCX, 558843051, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 187, 177, 171, 68, 79, 33], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 252, 187, 211], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 56178352, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 141, 187, 132, 248, 176, 54, 89, 3], OperandSize::Dword)
}

#[test]
fn vfmsub231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 77, 191, 187, 202], OperandSize::Qword)
}

#[test]
fn vfmsub231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231SS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 29, 131, 187, 44, 194], OperandSize::Qword)
}

