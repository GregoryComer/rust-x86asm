use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 234, 227], OperandSize::Dword)
}

#[test]
fn vpminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 452482943, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 234, 132, 200, 127, 87, 248, 26], OperandSize::Dword)
}

#[test]
fn vpminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 234, 205], OperandSize::Qword)
}

#[test]
fn vpminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 447084478, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 234, 20, 93, 190, 247, 165, 26], OperandSize::Qword)
}

#[test]
fn vpminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 234, 197], OperandSize::Dword)
}

#[test]
fn vpminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ECX, 679628163, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 234, 145, 131, 77, 130, 40], OperandSize::Dword)
}

#[test]
fn vpminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 234, 192], OperandSize::Qword)
}

#[test]
fn vpminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1153150468, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 234, 148, 193, 4, 174, 187, 68], OperandSize::Qword)
}

#[test]
fn vpminsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 137, 234, 217], OperandSize::Dword)
}

#[test]
fn vpminsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 655761747, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 234, 169, 83, 33, 22, 39], OperandSize::Dword)
}

#[test]
fn vpminsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 45, 140, 234, 234], OperandSize::Qword)
}

#[test]
fn vpminsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1329625871, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 21, 134, 234, 28, 157, 15, 123, 64, 79], OperandSize::Qword)
}

#[test]
fn vpminsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 234, 244], OperandSize::Dword)
}

#[test]
fn vpminsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 171, 234, 4, 87], OperandSize::Dword)
}

#[test]
fn vpminsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 5, 174, 234, 198], OperandSize::Qword)
}

#[test]
fn vpminsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 22148962, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 234, 180, 222, 98, 247, 81, 1], OperandSize::Qword)
}

#[test]
fn vpminsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 203, 234, 230], OperandSize::Dword)
}

#[test]
fn vpminsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 816397840, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 206, 234, 148, 136, 16, 62, 169, 48], OperandSize::Dword)
}

#[test]
fn vpminsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 21, 204, 234, 248], OperandSize::Qword)
}

#[test]
fn vpminsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 453680139, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 204, 234, 172, 153, 11, 156, 10, 27], OperandSize::Qword)
}

