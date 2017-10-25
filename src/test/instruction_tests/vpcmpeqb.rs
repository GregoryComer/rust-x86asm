use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 116, 209], OperandSize::Dword)
}

fn vpcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 2013310535, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 116, 182, 71, 174, 0, 120], OperandSize::Dword)
}

fn vpcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 116, 197], OperandSize::Qword)
}

fn vpcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 116, 4, 219], OperandSize::Qword)
}

fn vpcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 116, 239], OperandSize::Dword)
}

fn vpcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 116, 52, 83], OperandSize::Dword)
}

fn vpcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 116, 203], OperandSize::Qword)
}

fn vpcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 2067761349, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 116, 178, 197, 136, 63, 123], OperandSize::Qword)
}

fn vpcmpeqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 9, 116, 252], OperandSize::Dword)
}

fn vpcmpeqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 1310843927, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 10, 116, 191, 23, 228, 33, 78], OperandSize::Dword)
}

fn vpcmpeqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 29, 13, 116, 205], OperandSize::Qword)
}

fn vpcmpeqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM31)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 5, 3, 116, 51], OperandSize::Qword)
}

fn vpcmpeqb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 45, 116, 240], OperandSize::Dword)
}

fn vpcmpeqb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 42, 116, 28, 152], OperandSize::Dword)
}

fn vpcmpeqb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 5, 46, 116, 216], OperandSize::Qword)
}

fn vpcmpeqb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1691411322, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 45, 45, 116, 28, 221, 122, 227, 208, 100], OperandSize::Qword)
}

fn vpcmpeqb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 74, 116, 254], OperandSize::Dword)
}

fn vpcmpeqb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1156541262, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 78, 116, 20, 221, 78, 107, 239, 68], OperandSize::Dword)
}

fn vpcmpeqb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 61, 74, 116, 241], OperandSize::Qword)
}

fn vpcmpeqb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 13, 79, 116, 46], OperandSize::Qword)
}

