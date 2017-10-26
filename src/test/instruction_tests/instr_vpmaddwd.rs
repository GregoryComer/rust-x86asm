use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 245, 197], OperandSize::Dword)
}

#[test]
fn vpmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 245, 44, 73], OperandSize::Dword)
}

#[test]
fn vpmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 245, 217], OperandSize::Qword)
}

#[test]
fn vpmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 245, 52, 150], OperandSize::Qword)
}

#[test]
fn vpmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 245, 204], OperandSize::Dword)
}

#[test]
fn vpmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 2077204586, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 245, 180, 184, 106, 160, 207, 123], OperandSize::Dword)
}

#[test]
fn vpmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 245, 207], OperandSize::Qword)
}

#[test]
fn vpmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 245, 12, 152], OperandSize::Qword)
}

#[test]
fn vpmaddwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 245, 234], OperandSize::Dword)
}

#[test]
fn vpmaddwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 245, 30], OperandSize::Dword)
}

#[test]
fn vpmaddwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 53, 142, 245, 229], OperandSize::Qword)
}

#[test]
fn vpmaddwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 815649675, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 5, 130, 245, 148, 183, 139, 211, 157, 48], OperandSize::Qword)
}

#[test]
fn vpmaddwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 245, 246], OperandSize::Dword)
}

#[test]
fn vpmaddwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1650618267, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 175, 245, 164, 143, 155, 111, 98, 98], OperandSize::Dword)
}

#[test]
fn vpmaddwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 13, 167, 245, 244], OperandSize::Qword)
}

#[test]
fn vpmaddwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 57775365, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 37, 171, 245, 132, 142, 5, 149, 113, 3], OperandSize::Qword)
}

#[test]
fn vpmaddwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 206, 245, 230], OperandSize::Dword)
}

#[test]
fn vpmaddwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 245, 28, 144], OperandSize::Dword)
}

#[test]
fn vpmaddwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 37, 193, 245, 219], OperandSize::Qword)
}

#[test]
fn vpmaddwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 2146045488, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 21, 205, 245, 164, 198, 48, 14, 234, 127], OperandSize::Qword)
}

