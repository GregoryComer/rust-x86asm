use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 194], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 12, 211], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 199], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RSI, 2143437513, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 134, 201, 66, 194, 127], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 244], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(ECX, 1875541829, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 185, 69, 127, 202, 111], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 202], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 60, 159], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 32, 218], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1343311914, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 32, 60, 85, 42, 80, 17, 80], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 32, 239], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 140, 32, 4, 211], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 32, 244], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EAX, 182431785, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 32, 168, 41, 176, 223, 10], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 125, 175, 32, 218], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM24)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1700944498, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 175, 32, 4, 205, 114, 90, 98, 101], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 32, 229], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1440076871, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 32, 156, 112, 71, 212, 213, 85], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 207, 32, 206], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectDisplaced(RDX, 150388851, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 206, 32, 186, 115, 192, 246, 8], OperandSize::Qword)
}

