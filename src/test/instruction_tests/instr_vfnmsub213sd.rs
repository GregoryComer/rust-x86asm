use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 175, 236], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1326612191, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 175, 60, 85, 223, 126, 18, 79], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 175, 196], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1255346215, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 175, 20, 253, 39, 16, 211, 74], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 254, 175, 235], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 143, 175, 28, 90], OperandSize::Dword)
}

#[test]
fn vfnmsub213sd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 253, 246, 175, 209], OperandSize::Qword)
}

#[test]
fn vfnmsub213sd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RDX, 2126014822, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 181, 135, 175, 146, 102, 105, 184, 126], OperandSize::Qword)
}

