use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vplzcntd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 68, 195], OperandSize::Dword)
}

#[test]
fn vplzcntd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 68, 20, 251], OperandSize::Dword)
}

#[test]
fn vplzcntd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ESI, 498298933, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 154, 68, 150, 53, 112, 179, 29], OperandSize::Dword)
}

#[test]
fn vplzcntd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 68, 236], OperandSize::Qword)
}

#[test]
fn vplzcntd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 243001332, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 142, 68, 4, 213, 244, 231, 123, 14], OperandSize::Qword)
}

#[test]
fn vplzcntd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM9)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 731733052, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 159, 68, 140, 159, 60, 92, 157, 43], OperandSize::Qword)
}

#[test]
fn vplzcntd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 68, 240], OperandSize::Dword)
}

#[test]
fn vplzcntd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1395476120, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 68, 164, 200, 152, 70, 45, 83], OperandSize::Dword)
}

#[test]
fn vplzcntd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 68, 14], OperandSize::Dword)
}

#[test]
fn vplzcntd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 125, 169, 68, 248], OperandSize::Qword)
}

#[test]
fn vplzcntd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 68, 60, 86], OperandSize::Qword)
}

#[test]
fn vplzcntd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM11)), operand2: Some(IndirectDisplaced(RAX, 362045317, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 185, 68, 152, 133, 95, 148, 21], OperandSize::Qword)
}

#[test]
fn vplzcntd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 68, 226], OperandSize::Dword)
}

#[test]
fn vplzcntd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 68, 11], OperandSize::Dword)
}

#[test]
fn vplzcntd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 68, 33], OperandSize::Dword)
}

#[test]
fn vplzcntd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 125, 207, 68, 251], OperandSize::Qword)
}

#[test]
fn vplzcntd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 206, 68, 12, 142], OperandSize::Qword)
}

#[test]
fn vplzcntd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 220, 68, 44, 122], OperandSize::Qword)
}

