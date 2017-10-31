use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 243, 127], OperandSize::Dword)
}

#[test]
fn vpsllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 115, 243, 112], OperandSize::Qword)
}

#[test]
fn vpsllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 240, 49], OperandSize::Dword)
}

#[test]
fn vpsllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 115, 241, 84], OperandSize::Qword)
}

#[test]
fn vpsllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 243, 240], OperandSize::Dword)
}

#[test]
fn vpsllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 2089137951, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 243, 185, 31, 183, 133, 124], OperandSize::Dword)
}

#[test]
fn vpsllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 243, 225], OperandSize::Qword)
}

#[test]
fn vpsllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 812450729, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 243, 156, 144, 169, 3, 109, 48], OperandSize::Qword)
}

#[test]
fn vpsllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 243, 211], OperandSize::Dword)
}

#[test]
fn vpsllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 519054565, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 243, 12, 69, 229, 36, 240, 30], OperandSize::Dword)
}

#[test]
fn vpsllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 243, 231], OperandSize::Qword)
}

#[test]
fn vpsllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 504770819, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 243, 172, 127, 3, 49, 22, 30], OperandSize::Qword)
}

#[test]
fn vpsllq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 139, 243, 254], OperandSize::Dword)
}

#[test]
fn vpsllq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1766144879, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 139, 243, 164, 217, 111, 59, 69, 105], OperandSize::Dword)
}

#[test]
fn vpsllq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 205, 132, 243, 202], OperandSize::Qword)
}

#[test]
fn vpsllq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 245, 132, 243, 44, 179], OperandSize::Qword)
}

#[test]
fn vpsllq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 170, 243, 202], OperandSize::Dword)
}

#[test]
fn vpsllq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 282482173, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 173, 243, 60, 253, 253, 85, 214, 16], OperandSize::Dword)
}

#[test]
fn vpsllq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 141, 167, 243, 201], OperandSize::Qword)
}

#[test]
fn vpsllq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 165, 161, 243, 33], OperandSize::Qword)
}

#[test]
fn vpsllq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 202, 243, 233], OperandSize::Dword)
}

#[test]
fn vpsllq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 548062498, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 201, 243, 52, 221, 34, 197, 170, 32], OperandSize::Dword)
}

#[test]
fn vpsllq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 149, 196, 243, 255], OperandSize::Qword)
}

#[test]
fn vpsllq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 141, 202, 243, 4, 138], OperandSize::Qword)
}

