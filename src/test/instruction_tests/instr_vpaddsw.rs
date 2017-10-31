use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 237, 228], OperandSize::Dword)
}

#[test]
fn vpaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 237, 4, 115], OperandSize::Dword)
}

#[test]
fn vpaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 237, 247], OperandSize::Qword)
}

#[test]
fn vpaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 237, 4, 135], OperandSize::Qword)
}

#[test]
fn vpaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 237, 197], OperandSize::Dword)
}

#[test]
fn vpaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 237, 36, 194], OperandSize::Dword)
}

#[test]
fn vpaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 237, 209], OperandSize::Qword)
}

#[test]
fn vpaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1335390446, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 237, 44, 149, 238, 112, 152, 79], OperandSize::Qword)
}

#[test]
fn vpaddsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 142, 237, 234], OperandSize::Dword)
}

#[test]
fn vpaddsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1805796029, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 138, 237, 60, 245, 189, 66, 162, 107], OperandSize::Dword)
}

#[test]
fn vpaddsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 117, 134, 237, 200], OperandSize::Qword)
}

#[test]
fn vpaddsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1517720726, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 85, 132, 237, 28, 245, 150, 148, 118, 90], OperandSize::Qword)
}

#[test]
fn vpaddsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 237, 232], OperandSize::Dword)
}

#[test]
fn vpaddsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 173, 237, 40], OperandSize::Dword)
}

#[test]
fn vpaddsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 21, 173, 237, 248], OperandSize::Qword)
}

#[test]
fn vpaddsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 390724308, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 45, 169, 237, 172, 135, 212, 250, 73, 23], OperandSize::Qword)
}

#[test]
fn vpaddsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 203, 237, 240], OperandSize::Dword)
}

#[test]
fn vpaddsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 237, 4, 88], OperandSize::Dword)
}

#[test]
fn vpaddsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 29, 196, 237, 206], OperandSize::Qword)
}

#[test]
fn vpaddsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 109, 197, 237, 8], OperandSize::Qword)
}

