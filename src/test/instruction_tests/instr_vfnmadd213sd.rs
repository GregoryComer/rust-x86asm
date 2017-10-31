use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 173, 215], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1610841368, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 173, 4, 125, 24, 125, 3, 96], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 173, 229], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 173, 34], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 191, 173, 243], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 228661954, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 139, 173, 28, 213, 194, 26, 161, 13], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 133, 146, 173, 244], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RDI, 20738936, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 133, 142, 173, 143, 120, 115, 60, 1], OperandSize::Qword)
}

