use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 4, 228], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 148686773, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 4, 172, 154, 181, 199, 220, 8], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 4, 196], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1604223392, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 4, 132, 147, 160, 129, 158, 95], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 4, 251], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 4, 36, 64], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 4, 196], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDI, 1286263798, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 4, 183, 246, 211, 170, 76], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 4, 195], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1148768696, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 4, 20, 69, 184, 209, 120, 68], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 37, 142, 4, 223], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 473782928, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 37, 131, 4, 36, 157, 144, 90, 61, 28], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 4, 196], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 169, 4, 28, 142], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 53, 163, 4, 208], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 29, 175, 4, 41], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 202, 4, 249], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1661202565, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 204, 4, 164, 242, 133, 240, 3, 99], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 37, 202, 4, 251], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 2011900034, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 85, 198, 4, 148, 79, 130, 40, 235, 119], OperandSize::Qword)
}

