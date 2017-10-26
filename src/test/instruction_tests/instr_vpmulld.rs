use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 64, 223], OperandSize::Dword)
}

#[test]
fn vpmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 64, 44, 137], OperandSize::Dword)
}

#[test]
fn vpmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 64, 197], OperandSize::Qword)
}

#[test]
fn vpmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1816301525, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 64, 52, 117, 213, 143, 66, 108], OperandSize::Qword)
}

#[test]
fn vpmulld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 64, 193], OperandSize::Dword)
}

#[test]
fn vpmulld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 64, 28, 89], OperandSize::Dword)
}

#[test]
fn vpmulld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 64, 194], OperandSize::Qword)
}

#[test]
fn vpmulld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 64, 0], OperandSize::Qword)
}

#[test]
fn vpmulld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 140, 64, 192], OperandSize::Dword)
}

#[test]
fn vpmulld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 64, 49], OperandSize::Dword)
}

#[test]
fn vpmulld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 157, 64, 44, 185], OperandSize::Dword)
}

#[test]
fn vpmulld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 29, 140, 64, 213], OperandSize::Qword)
}

#[test]
fn vpmulld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 129, 64, 52, 179], OperandSize::Qword)
}

#[test]
fn vpmulld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 77, 154, 64, 2], OperandSize::Qword)
}

#[test]
fn vpmulld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 64, 250], OperandSize::Dword)
}

#[test]
fn vpmulld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1245354630, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 170, 64, 60, 197, 134, 154, 58, 74], OperandSize::Dword)
}

#[test]
fn vpmulld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1321381891, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 189, 64, 188, 185, 3, 176, 194, 78], OperandSize::Dword)
}

#[test]
fn vpmulld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 85, 165, 64, 193], OperandSize::Qword)
}

#[test]
fn vpmulld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1039867207, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 77, 171, 64, 180, 177, 71, 29, 251, 61], OperandSize::Qword)
}

#[test]
fn vpmulld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 109, 185, 64, 12, 206], OperandSize::Qword)
}

#[test]
fn vpmulld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 201, 64, 251], OperandSize::Dword)
}

#[test]
fn vpmulld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 64, 52, 143], OperandSize::Dword)
}

#[test]
fn vpmulld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 16207955, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 219, 64, 28, 117, 83, 80, 247, 0], OperandSize::Dword)
}

#[test]
fn vpmulld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 29, 193, 64, 253], OperandSize::Qword)
}

#[test]
fn vpmulld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 901351981, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 77, 195, 64, 172, 121, 45, 138, 185, 53], OperandSize::Qword)
}

#[test]
fn vpmulld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectDisplaced(RDI, 2057668151, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 21, 223, 64, 191, 55, 134, 165, 122], OperandSize::Qword)
}

