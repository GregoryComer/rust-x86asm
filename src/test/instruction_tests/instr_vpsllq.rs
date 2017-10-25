use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 115, 247, 32], OperandSize::Dword)
}

#[test]
fn vpsllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 115, 245, 126], OperandSize::Qword)
}

#[test]
fn vpsllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 115, 242, 68], OperandSize::Dword)
}

#[test]
fn vpsllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 115, 242, 43], OperandSize::Qword)
}

#[test]
fn vpsllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 243, 192], OperandSize::Dword)
}

#[test]
fn vpsllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 243, 57], OperandSize::Dword)
}

#[test]
fn vpsllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 243, 233], OperandSize::Qword)
}

#[test]
fn vpsllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 243, 1], OperandSize::Qword)
}

#[test]
fn vpsllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 243, 209], OperandSize::Dword)
}

#[test]
fn vpsllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 523478947, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 243, 60, 197, 163, 167, 51, 31], OperandSize::Dword)
}

#[test]
fn vpsllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 243, 223], OperandSize::Qword)
}

#[test]
fn vpsllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 510145812, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 243, 60, 69, 20, 53, 104, 30], OperandSize::Qword)
}

#[test]
fn vpsllq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 138, 243, 238], OperandSize::Dword)
}

#[test]
fn vpsllq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 140, 243, 4, 120], OperandSize::Dword)
}

#[test]
fn vpsllq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 237, 140, 243, 222], OperandSize::Qword)
}

#[test]
fn vpsllq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RCX, 375079097, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 189, 130, 243, 137, 185, 64, 91, 22], OperandSize::Qword)
}

#[test]
fn vpsllq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 169, 243, 245], OperandSize::Dword)
}

#[test]
fn vpsllq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 1496130762, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 172, 243, 150, 202, 36, 45, 89], OperandSize::Dword)
}

#[test]
fn vpsllq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 205, 166, 243, 217], OperandSize::Qword)
}

#[test]
fn vpsllq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RCX, 1961868807, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 181, 162, 243, 161, 7, 190, 239, 116], OperandSize::Qword)
}

#[test]
fn vpsllq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 207, 243, 252], OperandSize::Dword)
}

#[test]
fn vpsllq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1764777401, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 204, 243, 60, 181, 185, 93, 48, 105], OperandSize::Dword)
}

#[test]
fn vpsllq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 149, 205, 243, 234], OperandSize::Qword)
}

#[test]
fn vpsllq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RSI, 517219864, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 197, 205, 243, 150, 24, 38, 212, 30], OperandSize::Qword)
}

