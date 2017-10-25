use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 54, 200], OperandSize::Dword)
}

fn vpermq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 2095174173, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 173, 54, 148, 177, 29, 210, 225, 124], OperandSize::Dword)
}

fn vpermq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 867463442, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 191, 54, 142, 18, 113, 180, 51], OperandSize::Dword)
}

fn vpermq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 157, 163, 54, 240], OperandSize::Qword)
}

fn vpermq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RBX, 1344605686, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 157, 169, 54, 155, 246, 13, 37, 80], OperandSize::Qword)
}

fn vpermq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RAX, 1626541206, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 157, 187, 54, 168, 150, 12, 243, 96], OperandSize::Qword)
}

fn vpermq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 206, 54, 231], OperandSize::Dword)
}

fn vpermq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EBX, 8809527, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 203, 54, 179, 55, 108, 134, 0], OperandSize::Dword)
}

fn vpermq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1688653793, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 54, 36, 205, 225, 207, 166, 100], OperandSize::Dword)
}

fn vpermq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 245, 193, 54, 223], OperandSize::Qword)
}

fn vpermq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 629283579, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 149, 194, 54, 52, 117, 251, 26, 130, 37], OperandSize::Qword)
}

fn vpermq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RCX, 309050621, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 141, 209, 54, 145, 253, 188, 107, 18], OperandSize::Qword)
}

fn vpermq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 198, 52], OperandSize::Dword)
}

fn vpermq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 2147361012, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 12, 253, 244, 32, 254, 127, 100], OperandSize::Dword)
}

fn vpermq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 234, 113], OperandSize::Qword)
}

fn vpermq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 0, 44, 200, 10], OperandSize::Qword)
}

fn vpermq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 0, 206, 100], OperandSize::Dword)
}

fn vpermq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(ESI, 1583898685, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 170, 0, 142, 61, 96, 104, 94, 110], OperandSize::Dword)
}

fn vpermq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 287533376, Some(OperandSize::Qword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 189, 0, 4, 197, 64, 105, 35, 17, 126], OperandSize::Dword)
}

fn vpermq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM14)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 211, 253, 173, 0, 206, 97], OperandSize::Qword)
}

fn vpermq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 253, 173, 0, 20, 75, 80], OperandSize::Qword)
}

fn vpermq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1221193034, Some(OperandSize::Qword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 190, 0, 148, 199, 74, 237, 201, 72, 85], OperandSize::Qword)
}

fn vpermq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 0, 246, 39], OperandSize::Dword)
}

fn vpermq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 198848143, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 0, 4, 117, 143, 46, 218, 11, 72], OperandSize::Dword)
}

fn vpermq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 218, 0, 12, 127, 3], OperandSize::Dword)
}

fn vpermq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM16)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 51, 253, 205, 0, 208, 55], OperandSize::Qword)
}

fn vpermq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 253, 202, 0, 12, 211, 32], OperandSize::Qword)
}

fn vpermq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMQ, operand1: Some(Direct(ZMM15)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 223, 0, 63, 0], OperandSize::Qword)
}

