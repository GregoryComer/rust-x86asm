use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 228, 214], OperandSize::Dword)
}

#[test]
fn vpmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1400261292, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 228, 156, 129, 172, 74, 118, 83], OperandSize::Dword)
}

#[test]
fn vpmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 228, 253], OperandSize::Qword)
}

#[test]
fn vpmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1386503241, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 228, 188, 144, 73, 92, 164, 82], OperandSize::Qword)
}

#[test]
fn vpmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 228, 242], OperandSize::Dword)
}

#[test]
fn vpmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 2128610634, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 228, 28, 149, 74, 5, 224, 126], OperandSize::Dword)
}

#[test]
fn vpmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 228, 254], OperandSize::Qword)
}

#[test]
fn vpmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1122027111, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 228, 148, 248, 103, 198, 224, 66], OperandSize::Qword)
}

#[test]
fn vpmulhuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 228, 220], OperandSize::Dword)
}

#[test]
fn vpmulhuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1959802786, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 228, 188, 88, 162, 55, 208, 116], OperandSize::Dword)
}

#[test]
fn vpmulhuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 29, 140, 228, 207], OperandSize::Qword)
}

#[test]
fn vpmulhuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1117032251, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 37, 139, 228, 140, 138, 59, 143, 148, 66], OperandSize::Qword)
}

#[test]
fn vpmulhuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 228, 235], OperandSize::Dword)
}

#[test]
fn vpmulhuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1514914195, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 172, 228, 172, 179, 147, 193, 75, 90], OperandSize::Dword)
}

#[test]
fn vpmulhuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 85, 166, 228, 233], OperandSize::Qword)
}

#[test]
fn vpmulhuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 228, 12, 120], OperandSize::Qword)
}

#[test]
fn vpmulhuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 206, 228, 220], OperandSize::Dword)
}

#[test]
fn vpmulhuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDI, 2033334639, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 202, 228, 143, 111, 57, 50, 121], OperandSize::Dword)
}

#[test]
fn vpmulhuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 69, 195, 228, 236], OperandSize::Qword)
}

#[test]
fn vpmulhuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 913162850, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 77, 202, 228, 20, 245, 98, 194, 109, 54], OperandSize::Qword)
}

