use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcompressps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 138, 243], OperandSize::Dword)
}

#[test]
fn vcompressps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectDisplaced(EDX, 1614242801, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 138, 186, 241, 99, 55, 96], OperandSize::Dword)
}

#[test]
fn vcompressps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 138, 254], OperandSize::Qword)
}

#[test]
fn vcompressps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 138, 28, 214], OperandSize::Qword)
}

#[test]
fn vcompressps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 138, 201], OperandSize::Dword)
}

#[test]
fn vcompressps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 138, 36, 150], OperandSize::Dword)
}

#[test]
fn vcompressps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 173, 138, 198], OperandSize::Qword)
}

#[test]
fn vcompressps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectDisplaced(RAX, 840654985, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 40, 138, 128, 137, 96, 27, 50], OperandSize::Qword)
}

#[test]
fn vcompressps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 138, 199], OperandSize::Dword)
}

#[test]
fn vcompressps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1646839543, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 138, 132, 66, 247, 198, 40, 98], OperandSize::Dword)
}

#[test]
fn vcompressps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 138, 204], OperandSize::Qword)
}

#[test]
fn vcompressps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 72, 138, 4, 88], OperandSize::Qword)
}

