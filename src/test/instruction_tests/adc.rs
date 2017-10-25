use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn adc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 203], OperandSize::Word)
}

fn adc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(SI, 98, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 84, 98], OperandSize::Word)
}

fn adc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 210], OperandSize::Dword)
}

fn adc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 27], OperandSize::Dword)
}

fn adc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 203], OperandSize::Qword)
}

fn adc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 23], OperandSize::Qword)
}

fn adc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 203], OperandSize::Qword)
}

fn adc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1466158695, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 156, 247, 103, 206, 99, 87], OperandSize::Qword)
}

fn adc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 212], OperandSize::Word)
}

fn adc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 241, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 161, 241, 0], OperandSize::Word)
}

fn adc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 222], OperandSize::Dword)
}

fn adc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 1431734988, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 156, 145, 204, 138, 86, 85], OperandSize::Dword)
}

fn adc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 207], OperandSize::Qword)
}

fn adc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RDX, 1656045734, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 138, 166, 64, 181, 98], OperandSize::Qword)
}

fn adc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 218], OperandSize::Word)
}

fn adc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(BX, 86, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 95, 86], OperandSize::Word)
}

fn adc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 250], OperandSize::Dword)
}

fn adc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 52, 126], OperandSize::Dword)
}

fn adc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 231], OperandSize::Qword)
}

fn adc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RAX, Two, 525517462, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 36, 69, 150, 194, 82, 31], OperandSize::Qword)
}

fn adc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 233], OperandSize::Qword)
}

fn adc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 38], OperandSize::Qword)
}

fn adc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 219], OperandSize::Word)
}

fn adc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 102, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 82, 102], OperandSize::Word)
}

fn adc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 219], OperandSize::Dword)
}

fn adc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1219114859, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 20, 133, 107, 55, 170, 72], OperandSize::Dword)
}

fn adc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 203], OperandSize::Qword)
}

fn adc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 20, 66], OperandSize::Qword)
}

fn adc_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[16, 202], OperandSize::Qword)
}

fn adc_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(RDX, 269466626, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[18, 146, 2, 188, 15, 16], OperandSize::Qword)
}

fn adc_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 246], OperandSize::Word)
}

fn adc_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 223, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 169, 223, 0], OperandSize::Word)
}

fn adc_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 227], OperandSize::Dword)
}

fn adc_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1299171562, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 188, 249, 234, 200, 111, 77], OperandSize::Dword)
}

fn adc_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 242], OperandSize::Qword)
}

fn adc_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 51], OperandSize::Qword)
}

fn adc_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 17, 210], OperandSize::Word)
}

fn adc_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 19, 29], OperandSize::Word)
}

fn adc_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 227], OperandSize::Dword)
}

fn adc_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1288829290, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 12, 253, 106, 249, 209, 76], OperandSize::Dword)
}

fn adc_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[17, 209], OperandSize::Qword)
}

fn adc_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RBX, 281388191, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[19, 179, 159, 164, 197, 16], OperandSize::Qword)
}

fn adc_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 17, 239], OperandSize::Qword)
}

fn adc_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 19, 28, 222], OperandSize::Qword)
}

fn adc_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 118], OperandSize::Word)
}

fn adc_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 14], OperandSize::Dword)
}

fn adc_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AL)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[20, 46], OperandSize::Qword)
}

fn adc_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(15434)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 74, 60], OperandSize::Word)
}

fn adc_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(25246)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 158, 98], OperandSize::Dword)
}

fn adc_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(AX)), operand2: Some(Literal16(17937)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 17, 70], OperandSize::Qword)
}

fn adc_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1256296074)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 21, 138, 142, 225, 74], OperandSize::Word)
}

fn adc_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1521135885)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 13, 177, 170, 90], OperandSize::Dword)
}

fn adc_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EAX)), operand2: Some(Literal32(353683290)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[21, 90, 199, 20, 21], OperandSize::Qword)
}

fn adc_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RAX)), operand2: Some(Literal32(551238288)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 21, 144, 58, 219, 32], OperandSize::Qword)
}

fn adc_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 210, 41], OperandSize::Word)
}

fn adc_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 10, Some(OperandSize::Byte), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 80, 10, 113], OperandSize::Word)
}

fn adc_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DL)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 210, 78], OperandSize::Dword)
}

fn adc_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 908603992, Some(OperandSize::Byte), None)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 148, 202, 88, 50, 40, 54, 67], OperandSize::Dword)
}

fn adc_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 209, 53], OperandSize::Qword)
}

fn adc_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 19, 24], OperandSize::Qword)
}

fn adc_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CL)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 209, 20], OperandSize::Qword)
}

fn adc_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 20, 143, 106], OperandSize::Qword)
}

fn adc_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(CX)), operand2: Some(Literal16(12974)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 209, 174, 50], OperandSize::Word)
}

fn adc_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal16(2937)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 16, 121, 11], OperandSize::Word)
}

fn adc_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(SI)), operand2: Some(Literal16(18155)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 214, 235, 70], OperandSize::Dword)
}

fn adc_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(ESI, 1729690082, Some(OperandSize::Word), None)), operand2: Some(Literal16(2061)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 150, 226, 249, 24, 103, 13, 8], OperandSize::Dword)
}

fn adc_67() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DX)), operand2: Some(Literal16(9188)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 210, 228, 35], OperandSize::Qword)
}

fn adc_68() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal16(21089)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 18, 97, 82], OperandSize::Qword)
}

fn adc_69() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBP)), operand2: Some(Literal32(1520076078)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 213, 46, 133, 154, 90], OperandSize::Word)
}

fn adc_70() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(SI, 18586, Some(OperandSize::Dword), None)), operand2: Some(Literal32(985544131)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 148, 154, 72, 195, 53, 190, 58], OperandSize::Word)
}

fn adc_71() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDX)), operand2: Some(Literal32(1888787251)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 210, 51, 155, 148, 112], OperandSize::Dword)
}

fn adc_72() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 904524025, Some(OperandSize::Dword), None)), operand2: Some(Literal32(401091513)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 148, 201, 249, 240, 233, 53, 185, 43, 232, 23], OperandSize::Dword)
}

fn adc_73() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EDI)), operand2: Some(Literal32(58893502)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 215, 190, 164, 130, 3], OperandSize::Qword)
}

fn adc_74() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(829951339)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 23, 107, 13, 120, 49], OperandSize::Qword)
}

fn adc_75() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RCX)), operand2: Some(Literal32(919622393)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 209, 249, 82, 208, 54], OperandSize::Qword)
}

fn adc_76() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 542503602, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1070339159)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 148, 128, 178, 242, 85, 32, 87, 20, 204, 63], OperandSize::Qword)
}

fn adc_77() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DI)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 215, 5], OperandSize::Word)
}

fn adc_78() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 19, 114], OperandSize::Word)
}

fn adc_79() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(BP)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 213, 96], OperandSize::Dword)
}

fn adc_80() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1783470237, Some(OperandSize::Word), None)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 20, 133, 157, 152, 77, 106, 95], OperandSize::Dword)
}

fn adc_81() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(DX)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 210, 19], OperandSize::Qword)
}

fn adc_82() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledDisplaced(RCX, Two, 148585916, Some(OperandSize::Word), None)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 20, 77, 188, 61, 219, 8, 20], OperandSize::Qword)
}

fn adc_83() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBX)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 211, 104], OperandSize::Word)
}

fn adc_84() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(BX, 72, Some(OperandSize::Dword), None)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 87, 72, 61], OperandSize::Word)
}

fn adc_85() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(ESP)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 212, 89], OperandSize::Dword)
}

fn adc_86() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 18, 105], OperandSize::Dword)
}

fn adc_87() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(EBP)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 213, 24], OperandSize::Qword)
}

fn adc_88() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectDisplaced(RCX, 114660135, Some(OperandSize::Dword), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 145, 39, 147, 213, 6, 38], OperandSize::Qword)
}

fn adc_89() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(Direct(RSI)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 214, 122], OperandSize::Qword)
}

fn adc_90() {
    run_test(&Instruction { mnemonic: Mnemonic::ADC, operand1: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 20, 71, 7], OperandSize::Qword)
}

