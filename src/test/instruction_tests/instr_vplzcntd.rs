use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vplzcntd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 68, 198], OperandSize::Dword)
}

#[test]
fn vplzcntd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 1001397203, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 68, 167, 211, 27, 176, 59], OperandSize::Dword)
}

#[test]
fn vplzcntd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 157, 68, 22], OperandSize::Dword)
}

#[test]
fn vplzcntd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 68, 200], OperandSize::Qword)
}

#[test]
fn vplzcntd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 142, 68, 36, 83], OperandSize::Qword)
}

#[test]
fn vplzcntd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 739717768, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 157, 68, 172, 215, 136, 50, 23, 44], OperandSize::Qword)
}

#[test]
fn vplzcntd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 68, 225], OperandSize::Dword)
}

#[test]
fn vplzcntd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1517388069, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 68, 52, 205, 37, 129, 113, 90], OperandSize::Dword)
}

#[test]
fn vplzcntd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 393937834, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 189, 68, 148, 203, 170, 3, 123, 23], OperandSize::Dword)
}

#[test]
fn vplzcntd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 169, 68, 207], OperandSize::Qword)
}

#[test]
fn vplzcntd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1837476535, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 174, 68, 36, 253, 183, 170, 133, 109], OperandSize::Qword)
}

#[test]
fn vplzcntd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM25)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 125, 190, 68, 12, 158], OperandSize::Qword)
}

#[test]
fn vplzcntd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 68, 243], OperandSize::Dword)
}

#[test]
fn vplzcntd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EAX, 353811414, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 68, 136, 214, 187, 22, 21], OperandSize::Dword)
}

#[test]
fn vplzcntd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 217, 68, 35], OperandSize::Dword)
}

#[test]
fn vplzcntd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 125, 206, 68, 198], OperandSize::Qword)
}

#[test]
fn vplzcntd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 201, 68, 52, 151], OperandSize::Qword)
}

#[test]
fn vplzcntd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 68, 8], OperandSize::Qword)
}

