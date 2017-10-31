use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 185, 196], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1568257860, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 185, 60, 117, 68, 183, 121, 93], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 185, 216], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 361429865, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 185, 147, 105, 251, 138, 21], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 155, 185, 211], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 937349529, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 185, 4, 133, 153, 209, 222, 55], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 21, 186, 185, 196], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 882133244, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 131, 185, 132, 199, 252, 72, 148, 52], OperandSize::Qword)
}

