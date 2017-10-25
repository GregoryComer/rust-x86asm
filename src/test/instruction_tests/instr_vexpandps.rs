use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexpandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 136, 194], OperandSize::Dword)
}

#[test]
fn vexpandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 136, 38], OperandSize::Dword)
}

#[test]
fn vexpandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 125, 143, 136, 193], OperandSize::Qword)
}

#[test]
fn vexpandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM8)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 138, 136, 1], OperandSize::Qword)
}

#[test]
fn vexpandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 136, 222], OperandSize::Dword)
}

#[test]
fn vexpandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1021707840, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 136, 20, 205, 64, 6, 230, 60], OperandSize::Dword)
}

#[test]
fn vexpandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 125, 175, 136, 228], OperandSize::Qword)
}

#[test]
fn vexpandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 136, 12, 81], OperandSize::Qword)
}

#[test]
fn vexpandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 136, 249], OperandSize::Dword)
}

#[test]
fn vexpandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 2025361729, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 136, 20, 117, 65, 145, 184, 120], OperandSize::Dword)
}

#[test]
fn vexpandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 125, 204, 136, 238], OperandSize::Qword)
}

#[test]
fn vexpandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 378361317, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 202, 136, 52, 197, 229, 85, 141, 22], OperandSize::Qword)
}

