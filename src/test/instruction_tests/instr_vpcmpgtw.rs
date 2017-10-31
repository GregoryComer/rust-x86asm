use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 101, 252], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 182937927, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 101, 148, 150, 71, 105, 231, 10], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 101, 193], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RCX, 1755004340, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 101, 177, 180, 61, 155, 104], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 101, 253], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1121370199, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 101, 148, 200, 87, 192, 214, 66], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 101, 227], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1933875565, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 101, 12, 245, 109, 153, 68, 115], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 12, 101, 239], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 9, 101, 58], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 125, 4, 101, 210], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 618396529, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 12, 101, 164, 183, 113, 251, 219, 36], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 44, 101, 214], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1106897843, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 44, 101, 188, 150, 179, 235, 249, 65], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 38, 101, 207], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 5, 39, 101, 18], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 78, 101, 221], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 220546934, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 75, 101, 60, 141, 118, 71, 37, 13], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 29, 73, 101, 240], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 912411514, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 5, 74, 101, 44, 245, 122, 75, 98, 54], OperandSize::Qword)
}

