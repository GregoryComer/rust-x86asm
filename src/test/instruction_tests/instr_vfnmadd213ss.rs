use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 173, 193], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 173, 52, 153], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 173, 204], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 173, 60, 126], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 220, 173, 228], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1293482139, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 173, 52, 245, 155, 248, 24, 77], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 29, 220, 173, 240], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RDI, 305699025, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 77, 131, 173, 191, 209, 152, 56, 18], OperandSize::Qword)
}

