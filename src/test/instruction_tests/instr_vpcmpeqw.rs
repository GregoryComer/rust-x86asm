use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 117, 199], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1648990385, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 117, 140, 218, 177, 152, 73, 98], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 117, 254], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1563023286, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 117, 36, 77, 182, 215, 41, 93], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 117, 211], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 117, 14], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 117, 194], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDX, 968324596, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 117, 162, 244, 117, 183, 57], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 10, 117, 222], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 349170623, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 14, 117, 139, 191, 235, 207, 20], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 125, 2, 117, 223], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 53, 6, 117, 28, 86], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 45, 117, 210], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1399181589, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 44, 117, 148, 153, 21, 209, 101, 83], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 117, 43, 117, 232], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RSI, 1991761728, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 34, 117, 142, 64, 223, 183, 118], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 76, 117, 229], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 634834334, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 78, 117, 20, 181, 158, 205, 214, 37], OperandSize::Dword)
}

#[test]
fn vpcmpeqw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 13, 77, 117, 238], OperandSize::Qword)
}

#[test]
fn vpcmpeqw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 472236436, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 53, 70, 117, 20, 117, 148, 193, 37, 28], OperandSize::Qword)
}

