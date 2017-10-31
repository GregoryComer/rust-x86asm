use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 202], OperandSize::Word)
}

#[test]
fn adc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 24], OperandSize::Word)
}

#[test]
fn adc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 209], OperandSize::Dword)
}

#[test]
fn adc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(EAX, 1494076145, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 152, 241, 202, 13, 89], OperandSize::Dword)
}

#[test]
fn adc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 217], OperandSize::Qword)
}

#[test]
fn adc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 28, 198], OperandSize::Qword)
}

#[test]
fn adc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 217], OperandSize::Qword)
}

#[test]
fn adc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 1150380199, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 140, 255, 167, 104, 145, 68], OperandSize::Qword)
}

#[test]
fn adc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 244], OperandSize::Word)
}

#[test]
fn adc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Memory(15806, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 38, 190, 61], OperandSize::Word)
}

#[test]
fn adc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 225], OperandSize::Dword)
}

#[test]
fn adc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 57], OperandSize::Dword)
}

#[test]
fn adc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 213], OperandSize::Qword)
}

#[test]
fn adc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RAX, 1750103780, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 160, 228, 118, 80, 104], OperandSize::Qword)
}

#[test]
fn adc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 249], OperandSize::Word)
}

#[test]
fn adc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Memory(24539, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 22, 219, 95], OperandSize::Word)
}

#[test]
fn adc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 245], OperandSize::Dword)
}

#[test]
fn adc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 442144986, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 52, 221, 218, 152, 90, 26], OperandSize::Dword)
}

#[test]
fn adc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 255], OperandSize::Qword)
}

#[test]
fn adc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 509242374, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 140, 158, 6, 108, 90, 30], OperandSize::Qword)
}

#[test]
fn adc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 241], OperandSize::Qword)
}

#[test]
fn adc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 51], OperandSize::Qword)
}

#[test]
fn adc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 219], OperandSize::Word)
}

#[test]
fn adc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(SI, 138, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 140, 138, 0], OperandSize::Word)
}

#[test]
fn adc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 209], OperandSize::Dword)
}

#[test]
fn adc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 20, 200], OperandSize::Dword)
}

#[test]
fn adc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 201], OperandSize::Qword)
}

#[test]
fn adc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1877955621, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 12, 189, 37, 84, 239, 111], OperandSize::Qword)
}

#[test]
fn adc_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 210], OperandSize::Qword)
}

#[test]
fn adc_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RBX, 1019882420, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 139, 180, 43, 202, 60], OperandSize::Qword)
}

#[test]
fn adc_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 236], OperandSize::Word)
}

#[test]
fn adc_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 157, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 162, 157, 0], OperandSize::Word)
}

#[test]
fn adc_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 238], OperandSize::Dword)
}

#[test]
fn adc_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 55], OperandSize::Dword)
}

#[test]
fn adc_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 203], OperandSize::Qword)
}

#[test]
fn adc_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 35], OperandSize::Qword)
}

#[test]
fn adc_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 235], OperandSize::Word)
}

#[test]
fn adc_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 63], OperandSize::Word)
}

#[test]
fn adc_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 234], OperandSize::Dword)
}

#[test]
fn adc_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 28, 250], OperandSize::Dword)
}

#[test]
fn adc_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 211], OperandSize::Qword)
}

#[test]
fn adc_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 38], OperandSize::Qword)
}

#[test]
fn adc_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 221], OperandSize::Qword)
}

#[test]
fn adc_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 19, 19], OperandSize::Qword)
}

#[test]
fn adc_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 93], OperandSize::Word)
}

#[test]
fn adc_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 64], OperandSize::Dword)
}

#[test]
fn adc_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 44], OperandSize::Qword)
}

#[test]
fn adc_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(30393)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 185, 118], OperandSize::Word)
}

#[test]
fn adc_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(18991)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 47, 74], OperandSize::Dword)
}

#[test]
fn adc_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(15990)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 118, 62], OperandSize::Qword)
}

#[test]
fn adc_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(954976000)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 0, 199, 235, 56], OperandSize::Word)
}

#[test]
fn adc_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(677744531)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 147, 143, 101, 40], OperandSize::Dword)
}

#[test]
fn adc_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1549915175)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 39, 212, 97, 92], OperandSize::Qword)
}

#[test]
fn adc_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RAX)), operand2: Some(Literal32(2023901856)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 21, 160, 74, 162, 120], OperandSize::Qword)
}

#[test]
fn adc_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 211, 50], OperandSize::Word)
}

#[test]
fn adc_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(BX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 23, 105], OperandSize::Word)
}

#[test]
fn adc_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 209, 0], OperandSize::Dword)
}

#[test]
fn adc_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 20, 195, 123], OperandSize::Dword)
}

#[test]
fn adc_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 209, 67], OperandSize::Qword)
}

#[test]
fn adc_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1334684636, Some(OperandSize::Byte), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 148, 115, 220, 171, 141, 79, 107], OperandSize::Qword)
}

#[test]
fn adc_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 211, 104], OperandSize::Qword)
}

#[test]
fn adc_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RAX, 465289429, Some(OperandSize::Byte), None)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 144, 213, 192, 187, 27, 11], OperandSize::Qword)
}

#[test]
fn adc_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Literal16(13781)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 212, 213, 53], OperandSize::Word)
}

#[test]
fn adc_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 227, Some(OperandSize::Word), None)), operand2: Some(Literal16(27809)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 144, 227, 0, 161, 108], OperandSize::Word)
}

#[test]
fn adc_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(Literal16(8025)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 215, 89, 31], OperandSize::Dword)
}

#[test]
fn adc_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(EAX, 994287073, Some(OperandSize::Word), None)), operand2: Some(Literal16(17978)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 144, 225, 157, 67, 59, 58, 70], OperandSize::Dword)
}

#[test]
fn adc_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BX)), operand2: Some(Literal16(23015)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 211, 231, 89], OperandSize::Qword)
}

#[test]
fn adc_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: Some(Literal16(23473)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 19, 177, 91], OperandSize::Qword)
}

#[test]
fn adc_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Literal32(956737330)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 215, 50, 167, 6, 57], OperandSize::Word)
}

#[test]
fn adc_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 19564, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1885538210)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 146, 108, 76, 162, 7, 99, 112], OperandSize::Word)
}

#[test]
fn adc_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(Literal32(507413403)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 209, 155, 131, 62, 30], OperandSize::Dword)
}

#[test]
fn adc_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 965060725, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1758438168)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 148, 137, 117, 168, 133, 57, 24, 163, 207, 104], OperandSize::Dword)
}

#[test]
fn adc_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1492498328)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 213, 152, 183, 245, 88], OperandSize::Qword)
}

#[test]
fn adc_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1393342951)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 22, 231, 185, 12, 83], OperandSize::Qword)
}

#[test]
fn adc_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RBP)), operand2: Some(Literal32(2106170801)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 213, 177, 157, 137, 125], OperandSize::Qword)
}

#[test]
fn adc_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RDI, 1859346092, Some(OperandSize::Qword), None)), operand2: Some(Literal32(825870710)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 151, 172, 94, 211, 110, 118, 201, 57, 49], OperandSize::Qword)
}

#[test]
fn adc_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 213, 94], OperandSize::Word)
}

#[test]
fn adc_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 17967, Some(OperandSize::Word), None)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 145, 47, 70, 118], OperandSize::Word)
}

#[test]
fn adc_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 214, 88], OperandSize::Dword)
}

#[test]
fn adc_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 20, 208, 27], OperandSize::Dword)
}

#[test]
fn adc_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DX)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 210, 106], OperandSize::Qword)
}

#[test]
fn adc_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 18, 98], OperandSize::Qword)
}

#[test]
fn adc_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDX)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 210, 48], OperandSize::Word)
}

#[test]
fn adc_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 19, 24], OperandSize::Word)
}

#[test]
fn adc_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 214, 40], OperandSize::Dword)
}

#[test]
fn adc_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 20, 119, 17], OperandSize::Dword)
}

#[test]
fn adc_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 214, 32], OperandSize::Qword)
}

#[test]
fn adc_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1605882976, Some(OperandSize::Dword), None)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 20, 181, 96, 212, 183, 95, 123], OperandSize::Qword)
}

#[test]
fn adc_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RDX)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 210, 2], OperandSize::Qword)
}

#[test]
fn adc_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 19, 116], OperandSize::Qword)
}

