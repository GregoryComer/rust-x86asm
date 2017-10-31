use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2w_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 117, 197], OperandSize::Dword)
}

#[test]
fn vpermi2w_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 335757965, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 141, 117, 166, 141, 66, 3, 20], OperandSize::Dword)
}

#[test]
fn vpermi2w_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 229, 134, 117, 229], OperandSize::Qword)
}

#[test]
fn vpermi2w_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1022210961, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 229, 131, 117, 4, 149, 145, 179, 237, 60], OperandSize::Qword)
}

#[test]
fn vpermi2w_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 170, 117, 246], OperandSize::Dword)
}

#[test]
fn vpermi2w_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 117, 46], OperandSize::Dword)
}

#[test]
fn vpermi2w_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 253, 162, 117, 209], OperandSize::Qword)
}

#[test]
fn vpermi2w_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 237, 173, 117, 44, 192], OperandSize::Qword)
}

#[test]
fn vpermi2w_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 204, 117, 215], OperandSize::Dword)
}

#[test]
fn vpermi2w_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 201, 117, 52, 255], OperandSize::Dword)
}

#[test]
fn vpermi2w_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 229, 206, 117, 193], OperandSize::Qword)
}

#[test]
fn vpermi2w_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2W, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1224785, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 133, 193, 117, 132, 131, 81, 176, 18, 0], OperandSize::Qword)
}

