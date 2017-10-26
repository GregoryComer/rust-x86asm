use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 185, 232], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 597902955, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 185, 44, 197, 107, 70, 163, 35], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 185, 210], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1850574623, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 185, 60, 189, 31, 135, 77, 110], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 156, 185, 205], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 730933870, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 185, 185, 110, 42, 145, 43], OperandSize::Dword)
}

#[test]
fn vfmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 109, 145, 185, 193], OperandSize::Qword)
}

#[test]
fn vfmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 724620632, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 61, 131, 185, 44, 77, 88, 213, 48, 43], OperandSize::Qword)
}

