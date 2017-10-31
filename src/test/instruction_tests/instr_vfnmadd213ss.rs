use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 173, 199], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1497263992, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 173, 60, 141, 120, 111, 62, 89], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 173, 206], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1265665772, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 173, 4, 181, 236, 134, 112, 75], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 217, 173, 195], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1630002133, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 173, 44, 253, 213, 219, 39, 97], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 53, 155, 173, 237], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 109, 141, 173, 56], OperandSize::Qword)
}

