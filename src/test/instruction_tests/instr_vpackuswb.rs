use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 103, 248], OperandSize::Dword)
}

#[test]
fn vpackuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 103, 23], OperandSize::Dword)
}

#[test]
fn vpackuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 103, 245], OperandSize::Qword)
}

#[test]
fn vpackuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 1772575285, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 103, 183, 53, 90, 167, 105], OperandSize::Qword)
}

#[test]
fn vpackuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 103, 199], OperandSize::Dword)
}

#[test]
fn vpackuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 103, 57], OperandSize::Dword)
}

#[test]
fn vpackuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 103, 213], OperandSize::Qword)
}

#[test]
fn vpackuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RBX, 429217680, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 103, 139, 144, 87, 149, 25], OperandSize::Qword)
}

#[test]
fn vpackuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 103, 255], OperandSize::Dword)
}

#[test]
fn vpackuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 103, 38], OperandSize::Dword)
}

#[test]
fn vpackuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 117, 142, 103, 228], OperandSize::Qword)
}

#[test]
fn vpackuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RAX, 672943141, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 143, 103, 168, 37, 76, 28, 40], OperandSize::Qword)
}

#[test]
fn vpackuswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 172, 103, 229], OperandSize::Dword)
}

#[test]
fn vpackuswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1909830579, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 170, 103, 140, 154, 179, 179, 213, 113], OperandSize::Dword)
}

#[test]
fn vpackuswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 69, 163, 103, 241], OperandSize::Qword)
}

#[test]
fn vpackuswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 37, 166, 103, 28, 203], OperandSize::Qword)
}

#[test]
fn vpackuswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 204, 103, 253], OperandSize::Dword)
}

#[test]
fn vpackuswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 313877406, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 202, 103, 180, 120, 158, 99, 181, 18], OperandSize::Dword)
}

#[test]
fn vpackuswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 77, 206, 103, 210], OperandSize::Qword)
}

#[test]
fn vpackuswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 469977678, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 45, 201, 103, 28, 69, 78, 74, 3, 28], OperandSize::Qword)
}

