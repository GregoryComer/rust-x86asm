use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 113, 241, 48], OperandSize::Dword)
}

#[test]
fn vpsllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 113, 241, 4], OperandSize::Qword)
}

#[test]
fn vpsllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 113, 245, 25], OperandSize::Dword)
}

#[test]
fn vpsllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 113, 247, 55], OperandSize::Qword)
}

#[test]
fn vpsllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 113, 244, 27], OperandSize::Dword)
}

#[test]
fn vpsllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 113, 52, 121, 123], OperandSize::Dword)
}

#[test]
fn vpsllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM22)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 117, 132, 113, 246, 51], OperandSize::Qword)
}

#[test]
fn vpsllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 113, 52, 75, 13], OperandSize::Qword)
}

#[test]
fn vpsllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 113, 244, 102], OperandSize::Dword)
}

#[test]
fn vpsllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 175, 113, 52, 79, 30], OperandSize::Dword)
}

#[test]
fn vpsllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 45, 169, 113, 242, 10], OperandSize::Qword)
}

#[test]
fn vpsllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 113, 51, 41], OperandSize::Qword)
}

#[test]
fn vpsllw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 113, 244, 51], OperandSize::Dword)
}

#[test]
fn vpsllw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1657681430, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 202, 113, 52, 245, 22, 54, 206, 98, 33], OperandSize::Dword)
}

#[test]
fn vpsllw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM17)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 61, 199, 113, 241, 71], OperandSize::Qword)
}

#[test]
fn vpsllw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1003434005, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 53, 197, 113, 52, 245, 21, 48, 207, 59, 68], OperandSize::Qword)
}

#[test]
fn vpsllw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 241, 241], OperandSize::Dword)
}

#[test]
fn vpsllw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 241, 16], OperandSize::Dword)
}

#[test]
fn vpsllw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 241, 212], OperandSize::Qword)
}

#[test]
fn vpsllw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1922605893, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 241, 28, 189, 69, 163, 152, 114], OperandSize::Qword)
}

#[test]
fn vpsllw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 241, 249], OperandSize::Dword)
}

#[test]
fn vpsllw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 550808185, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 241, 153, 121, 170, 212, 32], OperandSize::Dword)
}

#[test]
fn vpsllw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 241, 244], OperandSize::Qword)
}

#[test]
fn vpsllw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 241, 20, 70], OperandSize::Qword)
}

#[test]
fn vpsllw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 241, 208], OperandSize::Dword)
}

#[test]
fn vpsllw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 140, 241, 20, 203], OperandSize::Dword)
}

#[test]
fn vpsllw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 69, 129, 241, 217], OperandSize::Qword)
}

#[test]
fn vpsllw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 61, 133, 241, 44, 199], OperandSize::Qword)
}

#[test]
fn vpsllw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 169, 241, 208], OperandSize::Dword)
}

#[test]
fn vpsllw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 357941245, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 241, 4, 149, 253, 191, 85, 21], OperandSize::Dword)
}

#[test]
fn vpsllw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 77, 163, 241, 202], OperandSize::Qword)
}

#[test]
fn vpsllw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 85, 172, 241, 8], OperandSize::Qword)
}

#[test]
fn vpsllw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 241, 201], OperandSize::Dword)
}

#[test]
fn vpsllw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 204, 241, 4, 127], OperandSize::Dword)
}

#[test]
fn vpsllw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 61, 196, 241, 248], OperandSize::Qword)
}

#[test]
fn vpsllw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 61, 203, 241, 32], OperandSize::Qword)
}

