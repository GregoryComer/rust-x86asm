use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 92, 193], OperandSize::Dword)
}

#[test]
fn vsubss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 92, 30], OperandSize::Dword)
}

#[test]
fn vsubss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 92, 243], OperandSize::Qword)
}

#[test]
fn vsubss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1764248544, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 92, 20, 157, 224, 75, 40, 105], OperandSize::Qword)
}

#[test]
fn vsubss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 110, 223, 92, 199], OperandSize::Dword)
}

#[test]
fn vsubss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 92, 55], OperandSize::Dword)
}

#[test]
fn vsubss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 46, 154, 92, 201], OperandSize::Qword)
}

#[test]
fn vsubss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 871074081, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 30, 143, 92, 164, 89, 33, 137, 235, 51], OperandSize::Qword)
}

