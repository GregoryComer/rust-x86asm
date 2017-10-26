use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 88, 213], OperandSize::Dword)
}

#[test]
fn vaddsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1722843356, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 88, 132, 243, 220, 128, 176, 102], OperandSize::Dword)
}

#[test]
fn vaddsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 88, 222], OperandSize::Qword)
}

#[test]
fn vaddsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 1990017029, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 88, 146, 5, 64, 157, 118], OperandSize::Qword)
}

#[test]
fn vaddsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 223, 88, 239], OperandSize::Dword)
}

#[test]
fn vaddsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 88, 25], OperandSize::Dword)
}

#[test]
fn vaddsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 191, 147, 88, 199], OperandSize::Qword)
}

#[test]
fn vaddsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 426801211, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 175, 133, 88, 52, 77, 59, 120, 112, 25], OperandSize::Qword)
}

