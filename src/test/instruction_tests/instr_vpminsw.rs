use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 234, 246], OperandSize::Dword)
}

#[test]
fn vpminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1466763393, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 234, 20, 181, 129, 8, 109, 87], OperandSize::Dword)
}

#[test]
fn vpminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 234, 201], OperandSize::Qword)
}

#[test]
fn vpminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 234, 34], OperandSize::Qword)
}

#[test]
fn vpminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 234, 196], OperandSize::Dword)
}

#[test]
fn vpminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 477270950, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 234, 28, 125, 166, 147, 114, 28], OperandSize::Dword)
}

#[test]
fn vpminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 234, 193], OperandSize::Qword)
}

#[test]
fn vpminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 234, 18], OperandSize::Qword)
}

#[test]
fn vpminsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 234, 199], OperandSize::Dword)
}

#[test]
fn vpminsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1975690493, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 234, 148, 64, 253, 164, 194, 117], OperandSize::Dword)
}

#[test]
fn vpminsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 131, 234, 229], OperandSize::Qword)
}

#[test]
fn vpminsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 21, 135, 234, 52, 246], OperandSize::Qword)
}

#[test]
fn vpminsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 172, 234, 226], OperandSize::Dword)
}

#[test]
fn vpminsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 169, 234, 12, 66], OperandSize::Dword)
}

#[test]
fn vpminsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 45, 166, 234, 247], OperandSize::Qword)
}

#[test]
fn vpminsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 234, 23], OperandSize::Qword)
}

#[test]
fn vpminsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 206, 234, 215], OperandSize::Dword)
}

#[test]
fn vpminsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1727465715, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 207, 234, 188, 208, 243, 8, 247, 102], OperandSize::Dword)
}

#[test]
fn vpminsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 109, 204, 234, 221], OperandSize::Qword)
}

#[test]
fn vpminsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 182598393, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 69, 205, 234, 36, 77, 249, 58, 226, 10], OperandSize::Qword)
}

