use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 236, 208], OperandSize::Dword)
}

#[test]
fn vpaddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1023785040, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 236, 172, 95, 80, 184, 5, 61], OperandSize::Dword)
}

#[test]
fn vpaddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 236, 254], OperandSize::Qword)
}

#[test]
fn vpaddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 236, 36, 83], OperandSize::Qword)
}

#[test]
fn vpaddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 236, 197], OperandSize::Dword)
}

#[test]
fn vpaddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1898623324, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 236, 44, 197, 92, 177, 42, 113], OperandSize::Dword)
}

#[test]
fn vpaddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 236, 215], OperandSize::Qword)
}

#[test]
fn vpaddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1190369561, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 236, 148, 246, 25, 153, 243, 70], OperandSize::Qword)
}

#[test]
fn vpaddsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 140, 236, 202], OperandSize::Dword)
}

#[test]
fn vpaddsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 236, 12, 153], OperandSize::Dword)
}

#[test]
fn vpaddsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 37, 138, 236, 236], OperandSize::Qword)
}

#[test]
fn vpaddsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1056595900, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 117, 133, 236, 148, 176, 188, 95, 250, 62], OperandSize::Qword)
}

#[test]
fn vpaddsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 236, 245], OperandSize::Dword)
}

#[test]
fn vpaddsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 236, 42], OperandSize::Dword)
}

#[test]
fn vpaddsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 53, 161, 236, 227], OperandSize::Qword)
}

#[test]
fn vpaddsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1774356471, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 13, 172, 236, 180, 146, 247, 135, 194, 105], OperandSize::Qword)
}

#[test]
fn vpaddsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 236, 243], OperandSize::Dword)
}

#[test]
fn vpaddsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 1575494115, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 205, 236, 184, 227, 33, 232, 93], OperandSize::Dword)
}

#[test]
fn vpaddsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 93, 205, 236, 211], OperandSize::Qword)
}

#[test]
fn vpaddsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDSB, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1290019218, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 101, 203, 236, 12, 141, 146, 33, 228, 76], OperandSize::Qword)
}

