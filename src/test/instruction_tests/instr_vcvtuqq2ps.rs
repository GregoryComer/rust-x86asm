use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtuqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 122, 220], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 122, 36, 138], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 255, 138, 122, 208], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM29)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 255, 137, 122, 42], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 122, 241], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 170, 122, 10], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 255, 175, 122, 201], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM22)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 255, 170, 122, 55], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 217, 122, 236], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 498640883, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 201, 122, 12, 205, 243, 167, 184, 29], OperandSize::Dword)
}

#[test]
fn vcvtuqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 255, 188, 122, 229], OperandSize::Qword)
}

#[test]
fn vcvtuqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 224655493, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 207, 122, 12, 157, 133, 248, 99, 13], OperandSize::Qword)
}

