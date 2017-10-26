use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 100, 198], OperandSize::Dword)
}

#[test]
fn vpblendmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 2050661515, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 100, 60, 117, 139, 156, 58, 122], OperandSize::Dword)
}

#[test]
fn vpblendmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 901823898, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 154, 100, 140, 178, 154, 189, 192, 53], OperandSize::Dword)
}

#[test]
fn vpblendmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 101, 129, 100, 201], OperandSize::Qword)
}

#[test]
fn vpblendmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RSI, 954020567, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 101, 129, 100, 190, 215, 50, 221, 56], OperandSize::Qword)
}

#[test]
fn vpblendmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 13, 153, 100, 4, 138], OperandSize::Qword)
}

#[test]
fn vpblendmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 170, 100, 199], OperandSize::Dword)
}

#[test]
fn vpblendmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 173, 100, 17], OperandSize::Dword)
}

#[test]
fn vpblendmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 219387206, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 185, 100, 28, 69, 70, 149, 19, 13], OperandSize::Dword)
}

#[test]
fn vpblendmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 29, 171, 100, 199], OperandSize::Qword)
}

#[test]
fn vpblendmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RSI, 2046562041, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 85, 173, 100, 182, 249, 14, 252, 121], OperandSize::Qword)
}

#[test]
fn vpblendmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RAX, 585230718, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 77, 189, 100, 184, 126, 233, 225, 34], OperandSize::Qword)
}

#[test]
fn vpblendmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 203, 100, 249], OperandSize::Dword)
}

#[test]
fn vpblendmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 205, 100, 44, 153], OperandSize::Dword)
}

#[test]
fn vpblendmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 222, 100, 52, 199], OperandSize::Dword)
}

#[test]
fn vpblendmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 13, 193, 100, 194], OperandSize::Qword)
}

#[test]
fn vpblendmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 29, 204, 100, 18], OperandSize::Qword)
}

#[test]
fn vpblendmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1466261279, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 85, 222, 100, 12, 253, 31, 95, 101, 87], OperandSize::Qword)
}

