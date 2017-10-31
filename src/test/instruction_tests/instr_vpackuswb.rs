use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 103, 192], OperandSize::Dword)
}

#[test]
fn vpackuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 402637472, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 103, 132, 126, 160, 194, 255, 23], OperandSize::Dword)
}

#[test]
fn vpackuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 103, 238], OperandSize::Qword)
}

#[test]
fn vpackuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 1079713737, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 103, 147, 201, 31, 91, 64], OperandSize::Qword)
}

#[test]
fn vpackuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 103, 229], OperandSize::Dword)
}

#[test]
fn vpackuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 103, 50], OperandSize::Dword)
}

#[test]
fn vpackuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 103, 206], OperandSize::Qword)
}

#[test]
fn vpackuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1514899895, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 103, 164, 154, 183, 137, 75, 90], OperandSize::Qword)
}

#[test]
fn vpackuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 103, 214], OperandSize::Dword)
}

#[test]
fn vpackuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 656422691, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 103, 151, 35, 55, 32, 39], OperandSize::Dword)
}

#[test]
fn vpackuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 101, 132, 103, 227], OperandSize::Qword)
}

#[test]
fn vpackuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 326115926, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 61, 139, 103, 4, 197, 86, 34, 112, 19], OperandSize::Qword)
}

#[test]
fn vpackuswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 103, 237], OperandSize::Dword)
}

#[test]
fn vpackuswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 590634935, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 169, 103, 60, 69, 183, 95, 52, 35], OperandSize::Dword)
}

#[test]
fn vpackuswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 109, 161, 103, 219], OperandSize::Qword)
}

#[test]
fn vpackuswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 522668548, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 93, 170, 103, 12, 141, 4, 74, 39, 31], OperandSize::Qword)
}

#[test]
fn vpackuswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 206, 103, 220], OperandSize::Dword)
}

#[test]
fn vpackuswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 103, 36, 87], OperandSize::Dword)
}

#[test]
fn vpackuswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 37, 197, 103, 214], OperandSize::Qword)
}

#[test]
fn vpackuswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 101, 206, 103, 12, 193], OperandSize::Qword)
}

