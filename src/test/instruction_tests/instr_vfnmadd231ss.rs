use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 189, 230], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 189, 44, 211], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 189, 227], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 124905063, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 189, 52, 133, 103, 230, 113, 7], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 251, 189, 195], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1301891014, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 189, 132, 144, 198, 71, 153, 77], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 93, 156, 189, 227], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 37, 134, 189, 52, 87], OperandSize::Qword)
}

