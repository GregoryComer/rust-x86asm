use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 238, 255], OperandSize::Dword)
}

#[test]
fn vpmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 238, 36, 67], OperandSize::Dword)
}

#[test]
fn vpmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 238, 206], OperandSize::Qword)
}

#[test]
fn vpmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 238, 4, 184], OperandSize::Qword)
}

#[test]
fn vpmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 238, 245], OperandSize::Dword)
}

#[test]
fn vpmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 561343179, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 238, 52, 245, 203, 106, 117, 33], OperandSize::Dword)
}

#[test]
fn vpmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 238, 209], OperandSize::Qword)
}

#[test]
fn vpmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1936701070, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 238, 148, 127, 142, 182, 111, 115], OperandSize::Qword)
}

#[test]
fn vpmaxsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 238, 236], OperandSize::Dword)
}

#[test]
fn vpmaxsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 598583030, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 142, 238, 188, 130, 246, 166, 173, 35], OperandSize::Dword)
}

#[test]
fn vpmaxsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 101, 133, 238, 221], OperandSize::Qword)
}

#[test]
fn vpmaxsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1122334326, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 77, 134, 238, 140, 134, 118, 118, 229, 66], OperandSize::Qword)
}

#[test]
fn vpmaxsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 171, 238, 231], OperandSize::Dword)
}

#[test]
fn vpmaxsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1373695234, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 173, 238, 148, 155, 2, 237, 224, 81], OperandSize::Dword)
}

#[test]
fn vpmaxsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 21, 171, 238, 248], OperandSize::Qword)
}

#[test]
fn vpmaxsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 71536245, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 61, 163, 238, 180, 89, 117, 142, 67, 4], OperandSize::Qword)
}

#[test]
fn vpmaxsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 204, 238, 203], OperandSize::Dword)
}

#[test]
fn vpmaxsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 413183199, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 203, 238, 20, 221, 223, 172, 160, 24], OperandSize::Dword)
}

#[test]
fn vpmaxsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 29, 203, 238, 203], OperandSize::Qword)
}

#[test]
fn vpmaxsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RDX, 1397125215, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 29, 202, 238, 130, 95, 112, 70, 83], OperandSize::Qword)
}

