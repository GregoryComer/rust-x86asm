use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 173, 230], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 2041295523, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 173, 172, 80, 163, 178, 171, 121], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 173, 213], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 173, 38], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 188, 173, 254], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 1003852787, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 173, 137, 243, 147, 213, 59], OperandSize::Dword)
}

#[test]
fn vfnmadd213ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 45, 191, 173, 224], OperandSize::Qword)
}

#[test]
fn vfnmadd213ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213SS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 93, 130, 173, 26], OperandSize::Qword)
}

