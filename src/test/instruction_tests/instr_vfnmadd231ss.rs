use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 189, 196], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1340502621, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 189, 60, 141, 93, 114, 230, 79], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 189, 244], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 1043993499, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 189, 185, 155, 19, 58, 62], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 223, 189, 240], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 1681593495, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 143, 189, 137, 151, 20, 59, 100], OperandSize::Dword)
}

#[test]
fn vfnmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 29, 222, 189, 243], OperandSize::Qword)
}

#[test]
fn vfnmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231SS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RBX, 640903109, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 77, 142, 189, 155, 197, 103, 51, 38], OperandSize::Qword)
}

