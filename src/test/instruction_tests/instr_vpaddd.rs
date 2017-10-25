use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 254, 235], OperandSize::Dword)
}

#[test]
fn vpaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 254, 39], OperandSize::Dword)
}

#[test]
fn vpaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 254, 237], OperandSize::Qword)
}

#[test]
fn vpaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 254, 14], OperandSize::Qword)
}

#[test]
fn vpaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 254, 217], OperandSize::Dword)
}

#[test]
fn vpaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1352224588, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 254, 172, 150, 76, 79, 153, 80], OperandSize::Dword)
}

#[test]
fn vpaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 254, 217], OperandSize::Qword)
}

#[test]
fn vpaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1418635376, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 254, 172, 187, 112, 168, 142, 84], OperandSize::Qword)
}

#[test]
fn vpaddd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 254, 195], OperandSize::Dword)
}

#[test]
fn vpaddd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 254, 62], OperandSize::Dword)
}

#[test]
fn vpaddd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 2107913696, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 158, 254, 182, 224, 53, 164, 125], OperandSize::Dword)
}

#[test]
fn vpaddd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 53, 143, 254, 204], OperandSize::Qword)
}

#[test]
fn vpaddd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1467312984, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 117, 141, 254, 52, 69, 88, 107, 117, 87], OperandSize::Qword)
}

#[test]
fn vpaddd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RBX, 1913012088, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 151, 254, 131, 120, 63, 6, 114], OperandSize::Qword)
}

