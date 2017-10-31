use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 64, 240], OperandSize::Dword)
}

#[test]
fn vpmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 884210228, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 64, 147, 52, 250, 179, 52], OperandSize::Dword)
}

#[test]
fn vpmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 64, 205], OperandSize::Qword)
}

#[test]
fn vpmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 64, 59], OperandSize::Qword)
}

#[test]
fn vpmulld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 64, 203], OperandSize::Dword)
}

#[test]
fn vpmulld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 64, 36, 95], OperandSize::Dword)
}

#[test]
fn vpmulld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 64, 233], OperandSize::Qword)
}

#[test]
fn vpmulld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 64, 18], OperandSize::Qword)
}

#[test]
fn vpmulld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 64, 254], OperandSize::Dword)
}

#[test]
fn vpmulld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1792075438, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 64, 140, 193, 174, 230, 208, 106], OperandSize::Dword)
}

#[test]
fn vpmulld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1134106327, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 153, 64, 140, 207, 215, 22, 153, 67], OperandSize::Dword)
}

#[test]
fn vpmulld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 125, 140, 64, 225], OperandSize::Qword)
}

#[test]
fn vpmulld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 549626246, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 61, 135, 64, 4, 205, 134, 161, 194, 32], OperandSize::Qword)
}

#[test]
fn vpmulld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 2110901070, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 37, 155, 64, 172, 201, 78, 203, 209, 125], OperandSize::Qword)
}

#[test]
fn vpmulld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 64, 217], OperandSize::Dword)
}

#[test]
fn vpmulld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 172, 64, 52, 82], OperandSize::Dword)
}

#[test]
fn vpmulld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1407852463, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 191, 64, 188, 201, 175, 31, 234, 83], OperandSize::Dword)
}

#[test]
fn vpmulld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 117, 162, 64, 197], OperandSize::Qword)
}

#[test]
fn vpmulld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 376918805, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 45, 174, 64, 44, 69, 21, 83, 119, 22], OperandSize::Qword)
}

#[test]
fn vpmulld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 109, 180, 64, 31], OperandSize::Qword)
}

#[test]
fn vpmulld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 204, 64, 233], OperandSize::Dword)
}

#[test]
fn vpmulld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1288189046, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 205, 64, 180, 134, 118, 52, 200, 76], OperandSize::Dword)
}

#[test]
fn vpmulld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 285432950, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 64, 140, 193, 118, 92, 3, 17], OperandSize::Dword)
}

#[test]
fn vpmulld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 93, 207, 64, 208], OperandSize::Qword)
}

#[test]
fn vpmulld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1021977782, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 85, 205, 64, 132, 73, 182, 36, 234, 60], OperandSize::Qword)
}

#[test]
fn vpmulld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 654516412, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 77, 213, 64, 44, 133, 188, 32, 3, 39], OperandSize::Qword)
}

