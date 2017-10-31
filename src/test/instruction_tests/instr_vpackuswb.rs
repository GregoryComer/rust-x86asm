use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 103, 218], OperandSize::Dword)
}

#[test]
fn vpackuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 790974738, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 103, 20, 181, 18, 81, 37, 47], OperandSize::Dword)
}

#[test]
fn vpackuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 103, 216], OperandSize::Qword)
}

#[test]
fn vpackuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 103, 12, 217], OperandSize::Qword)
}

#[test]
fn vpackuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 103, 213], OperandSize::Dword)
}

#[test]
fn vpackuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 103, 24], OperandSize::Dword)
}

#[test]
fn vpackuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 103, 230], OperandSize::Qword)
}

#[test]
fn vpackuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1552445883, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 103, 148, 182, 187, 113, 136, 92], OperandSize::Qword)
}

#[test]
fn vpackuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 141, 103, 216], OperandSize::Dword)
}

#[test]
fn vpackuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 383013062, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 103, 12, 205, 198, 80, 212, 22], OperandSize::Dword)
}

#[test]
fn vpackuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 45, 141, 103, 213], OperandSize::Qword)
}

#[test]
fn vpackuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1699843003, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 125, 133, 103, 172, 150, 187, 139, 81, 101], OperandSize::Qword)
}

#[test]
fn vpackuswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 103, 233], OperandSize::Dword)
}

#[test]
fn vpackuswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1528741329, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 173, 103, 36, 245, 209, 189, 30, 91], OperandSize::Dword)
}

#[test]
fn vpackuswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 93, 174, 103, 226], OperandSize::Qword)
}

#[test]
fn vpackuswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RBX, 1236609759, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 53, 165, 103, 131, 223, 42, 181, 73], OperandSize::Qword)
}

#[test]
fn vpackuswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 203, 103, 253], OperandSize::Dword)
}

#[test]
fn vpackuswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 115839850, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 203, 103, 132, 81, 106, 147, 231, 6], OperandSize::Dword)
}

#[test]
fn vpackuswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 101, 196, 103, 233], OperandSize::Qword)
}

#[test]
fn vpackuswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 5, 205, 103, 4, 178], OperandSize::Qword)
}

