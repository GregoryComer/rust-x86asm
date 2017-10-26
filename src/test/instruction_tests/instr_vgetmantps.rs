use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 142, 38, 228, 39], OperandSize::Dword)
}

#[test]
fn vgetmantps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 143, 38, 20, 207, 119], OperandSize::Dword)
}

#[test]
fn vgetmantps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 158, 38, 31, 85], OperandSize::Dword)
}

#[test]
fn vgetmantps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM31)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 131, 125, 141, 38, 231, 70], OperandSize::Qword)
}

#[test]
fn vgetmantps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM29)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 125, 139, 38, 41, 52], OperandSize::Qword)
}

#[test]
fn vgetmantps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM27)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 125, 159, 38, 31, 80], OperandSize::Qword)
}

#[test]
fn vgetmantps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 170, 38, 200, 50], OperandSize::Dword)
}

#[test]
fn vgetmantps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 521947250, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 38, 36, 85, 114, 72, 28, 31, 59], OperandSize::Dword)
}

#[test]
fn vgetmantps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EDI, 1227062736, Some(OperandSize::Dword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 188, 38, 167, 208, 125, 35, 73, 86], OperandSize::Dword)
}

#[test]
fn vgetmantps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM20)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 125, 170, 38, 244, 4], OperandSize::Qword)
}

#[test]
fn vgetmantps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1085901001, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 125, 172, 38, 140, 128, 201, 136, 185, 64, 91], OperandSize::Qword)
}

#[test]
fn vgetmantps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM15)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 125, 191, 38, 59, 99], OperandSize::Qword)
}

#[test]
fn vgetmantps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 157, 38, 212, 110], OperandSize::Dword)
}

#[test]
fn vgetmantps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(EDX, 1567359413, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 38, 154, 181, 1, 108, 93, 59], OperandSize::Dword)
}

#[test]
fn vgetmantps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EAX, 662344435, Some(OperandSize::Dword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 223, 38, 128, 243, 146, 122, 39, 8], OperandSize::Dword)
}

#[test]
fn vgetmantps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM17)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 125, 155, 38, 201, 80], OperandSize::Qword)
}

#[test]
fn vgetmantps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM29)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 125, 202, 38, 40, 127], OperandSize::Qword)
}

#[test]
fn vgetmantps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectDisplaced(RCX, 52814501, Some(OperandSize::Dword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 125, 219, 38, 161, 165, 226, 37, 3, 103], OperandSize::Qword)
}

