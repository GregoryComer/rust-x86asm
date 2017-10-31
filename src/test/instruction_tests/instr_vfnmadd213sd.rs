use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 173, 197], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1096488189, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 173, 140, 122, 253, 20, 91, 65], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 173, 215], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDX, 141057221, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 173, 146, 197, 92, 104, 8], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 223, 173, 250], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 3758178, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 173, 20, 213, 98, 88, 57, 0], OperandSize::Dword)
}

#[test]
fn vfnmadd213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 245, 178, 173, 203], OperandSize::Qword)
}

#[test]
fn vfnmadd213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 237, 131, 173, 22], OperandSize::Qword)
}

